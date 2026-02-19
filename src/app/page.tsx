import Link from "next/link";
import Image from "next/image";
import { BorderedBox } from "./_components/bordered-box";
import { FrostedBox } from "./_components/frosted-box";
import Header from "./_components/header";
import { CgolCanvas } from "./_components/cgol-canvas";
import FirstName from "~/assets/Jet.txt";

export const metadata = {
  title: "Jet Pham - Software Extremist",
  description: "Personal website of Jet Pham, a software extremist.",
};

export default async function Home() {
  return (
    <>
      <CgolCanvas />
      <main>
        <div className="flex flex-col items-center justify-start px-4">
          <FrostedBox className="my-[2ch] w-full max-w-[66.666667%] min-w-fit md:mt-[4ch]">
            <div className="flex flex-col items-center justify-center gap-[2ch] md:flex-row">
              <div className="order-1 flex flex-col items-center md:order-2">
                <Header content={FirstName} />
                <div className="mt-[2ch]">Software Extremist</div>
              </div>
              <div className="order-2 shrink-0 md:order-1">
                <Image
                  src="/jet.svg"
                  alt="Jet"
                  width={250}
                  height={250}
                  className="aspect-square w-full max-w-[250px] object-cover md:h-[263px] md:w-[175px] md:max-w-none"
                  priority
                />
              </div>
            </div>
            <BorderedBox label="Skills" className="mt-[2ch]">
              <div>Making crazy stuff</div>
            </BorderedBox>
            <BorderedBox label="Links">
              <ol>
                <li>
                  <Link
                    href="https://github.com/jetpham"
                    className="inline-flex items-center"
                  >
                    GitHub
                  </Link>
                </li>
                <li>
                  <Link
                    href="https://linkedin.com/in/jetpham"
                    className="inline-flex items-center"
                  >
                    LinkedIn
                  </Link>
                </li>
                <li>
                  <Link
                    href="https://bsky.app/profile/jetpham.com"
                    className="inline-flex items-center"
                  >
                    Bluesky
                  </Link>
                </li>
                <li>
                  <Link
                    href="https://x.com/jetpham5"
                    className="inline-flex items-center"
                  >
                    X (Twitter)
                  </Link>
                </li>
              </ol>
            </BorderedBox>
          </FrostedBox>
        </div>
      </main>
    </>
  );
}
