import Link from "next/link";
import Image from "next/image";
import { HydrateClient } from "~/trpc/server";
import { BorderedBox } from "./_components/bordered-box";
import { FrostedBox } from "./_components/frosted-box";
import Header from "./_components/header";
import { CgolCanvas } from "./_components/cgol-canvas";
import FirstName from "~/assets/Jet.txt";

export default async function Home() {
  return (
    <HydrateClient>
      <CgolCanvas />
      <main>
        <div className="flex flex-col items-center justify-start px-4">
          <FrostedBox className="mt-4 w-full max-w-[66.666667%] min-w-fit px-[calc(1.5ch-0.5px)] md:mt-16">
            <div className="flex flex-col items-center justify-center gap-[2ch] md:flex-row">
              <div className="order-1 flex flex-col items-center md:order-2">
                <Header content={FirstName} />
                <div className="mt-[3ch]">Software Extremist</div>
              </div>
              <div className="order-2 flex-shrink-0 px-[1ch] md:order-1">
                <div className="md:hidden w-full flex justify-center">
                  <div className="w-full max-w-[250px] aspect-square overflow-hidden">
                    <Image
                      src="/jet.svg"
                      alt="Jet"
                      width={250}
                      height={250}
                      className="w-full h-full object-cover"
                      priority
                    />
                  </div>
                </div>
                <Image
                  src="/jet.svg"
                  alt="Jet"
                  width={175}
                  height={263}
                  className="hidden md:block w-[175px] h-[263px]"
                  priority
                />
              </div>
            </div>
            <BorderedBox label="Skills">
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
                    X
            </Link>
                </li>
              </ol>
            </BorderedBox>
          </FrostedBox>
        </div>
      </main>
    </HydrateClient>
  );
}
